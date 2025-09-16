//! Stream processing utilities
//!
//! Public API for handling stdin/stdout, pipe operations, and buffered reading.
//! Leverages RSB's Stream type for robust text processing.

use super::error::{StreamError, StreamResult};
use rsb::prelude::Stream;
use std::io::{self, Read, Write, BufReader};
use std::time::Duration;

/// Configuration for stream processing
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Maximum buffer size in bytes
    pub max_buffer_size: usize,
    /// Whether to handle SIGPIPE gracefully
    pub handle_sigpipe: bool,
    /// Line ending style for output
    pub line_ending: LineEnding,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            max_buffer_size: 10 * 1024 * 1024, // 10MB default
            handle_sigpipe: true,
            line_ending: LineEnding::Unix,
        }
    }
}

/// Line ending styles
#[derive(Debug, Clone, Copy)]
pub enum LineEnding {
    Unix,    // \n
    Windows, // \r\n
    Mac,     // \r
}

impl LineEnding {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Mac => "\r",
        }
    }
}

/// Read all input from stdin with buffering and error handling
pub fn read_stdin() -> StreamResult<String> {
    read_stdin_with_config(&StreamConfig::default())
}

/// Read stdin with custom configuration
pub fn read_stdin_with_config(config: &StreamConfig) -> StreamResult<String> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut buffer = Vec::new();

    // Read with size limit
    reader.take(config.max_buffer_size as u64)
          .read_to_end(&mut buffer)
          .map_err(StreamError::from)?;

    // Convert to string with UTF-8 validation
    String::from_utf8(buffer).map_err(StreamError::from)
}

/// Write content to stdout with error handling
pub fn write_stdout(content: &str) -> StreamResult<()> {
    write_stdout_with_config(content, &StreamConfig::default())
}

/// Write to stdout with custom configuration
pub fn write_stdout_with_config(content: &str, config: &StreamConfig) -> StreamResult<()> {
    let mut stdout = io::stdout();

    // Handle SIGPIPE gracefully if configured
    if config.handle_sigpipe {
        if let Err(e) = stdout.write_all(content.as_bytes()) {
            if e.kind() == io::ErrorKind::BrokenPipe {
                // Gracefully handle pipe break (e.g., head command)
                return Ok(());
            }
            return Err(StreamError::from(e));
        }
    } else {
        stdout.write_all(content.as_bytes()).map_err(StreamError::from)?;
    }

    stdout.flush().map_err(StreamError::from)?;
    Ok(())
}

/// Create RSB Stream from stdin
pub fn stdin_to_stream() -> StreamResult<Stream> {
    let content = read_stdin()?;
    Ok(Stream::from_string(&content))
}

/// Write RSB Stream to stdout
pub fn stream_to_stdout(stream: &Stream) -> StreamResult<()> {
    write_stdout(&stream.to_string())
}

/// Process stdin through a function and write to stdout
pub fn pipe_transform<F>(transform: F) -> StreamResult<()>
where
    F: FnOnce(Stream) -> Stream,
{
    let input_stream = stdin_to_stream()?;
    let output_stream = transform(input_stream);
    stream_to_stdout(&output_stream)
}

/// Process stdin line by line with a function
pub fn pipe_lines<F>(line_processor: F) -> StreamResult<()>
where
    F: Fn(&str) -> String,
{
    pipe_transform(|stream| {
        stream.map(line_processor)
    })
}

/// Check if stdin has data available (non-blocking)
pub fn stdin_has_data() -> bool {
    use std::os::unix::io::AsRawFd;

    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Use select() to check if data is available
    unsafe {
        let mut fd_set: rsb::deps::libc::fd_set = std::mem::zeroed();
        rsb::deps::libc::FD_ZERO(&mut fd_set);
        rsb::deps::libc::FD_SET(fd, &mut fd_set);

        let mut timeout = rsb::deps::libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };

        rsb::deps::libc::select(fd + 1, &mut fd_set, std::ptr::null_mut(), std::ptr::null_mut(), &mut timeout) > 0
    }
}

/// Read from stdin with timeout
pub fn read_stdin_timeout(timeout: Duration) -> StreamResult<Option<String>> {
    use std::sync::mpsc;
    use std::thread;

    let (sender, receiver) = mpsc::channel();

    // Spawn thread to read stdin
    thread::spawn(move || {
        let result = read_stdin();
        let _ = sender.send(result);
    });

    // Wait for result with timeout
    match receiver.recv_timeout(timeout) {
        Ok(result) => result.map(Some),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            Err(StreamError::IoError("Reader thread disconnected".to_string()))
        }
    }
}

/// Create a processing pipeline that reads stdin -> processes -> writes stdout
pub fn create_pipeline() -> Pipeline {
    Pipeline::new()
}

/// Builder for stream processing pipelines
pub struct Pipeline {
    config: StreamConfig,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            config: StreamConfig::default(),
        }
    }

    pub fn with_config(mut self, config: StreamConfig) -> Self {
        self.config = config;
        self
    }

    pub fn max_buffer_size(mut self, size: usize) -> Self {
        self.config.max_buffer_size = size;
        self
    }

    pub fn handle_sigpipe(mut self, handle: bool) -> Self {
        self.config.handle_sigpipe = handle;
        self
    }

    pub fn line_ending(mut self, ending: LineEnding) -> Self {
        self.config.line_ending = ending;
        self
    }

    /// Execute the pipeline with a transform function
    pub fn execute<F>(self, transform: F) -> StreamResult<()>
    where
        F: FnOnce(Stream) -> Stream,
    {
        let input = read_stdin_with_config(&self.config)?;
        let stream = Stream::from_string(&input);
        let output_stream = transform(stream);
        write_stdout_with_config(&output_stream.to_string(), &self.config)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}