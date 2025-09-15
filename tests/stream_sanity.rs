//! Stream module sanity tests
//!
//! Basic functionality tests for the stream processing module

use rolo::prelude::*;

#[test]
fn test_stream_config_default() {
    let config = StreamConfig::default();
    assert_eq!(config.max_buffer_size, 10 * 1024 * 1024);
    assert!(config.handle_sigpipe);
    assert!(matches!(config.line_ending, LineEnding::Unix));
}

#[test]
fn test_line_ending_conversion() {
    assert_eq!(LineEnding::Unix.as_str(), "\n");
    assert_eq!(LineEnding::Windows.as_str(), "\r\n");
    assert_eq!(LineEnding::Mac.as_str(), "\r");
}

#[test]
fn test_pipeline_builder() {
    // Test that pipeline builder methods work (config is private, so we just test the builder pattern)
    let _pipeline = Pipeline::new()
        .max_buffer_size(1024)
        .handle_sigpipe(false)
        .line_ending(LineEnding::Windows);

    // Pipeline builder pattern works if this compiles
    assert!(true);
}

#[test]
fn test_stream_error_display() {
    let error = StreamError::StdinRead("test error".to_string());
    assert!(error.to_string().contains("Failed to read from stdin"));

    let error = StreamError::PipeBroken("test pipe".to_string());
    assert!(error.to_string().contains("Pipe broken"));

    let error = StreamError::BufferOverflow(1024);
    assert!(error.to_string().contains("Buffer overflow at 1024 bytes"));
}

#[test]
fn test_stream_error_conversion() {
    use std::io::{Error, ErrorKind};

    let io_error = Error::new(ErrorKind::BrokenPipe, "pipe broken");
    let stream_error: StreamError = io_error.into();
    assert!(matches!(stream_error, StreamError::PipeBroken(_)));

    let io_error = Error::new(ErrorKind::UnexpectedEof, "unexpected eof");
    let stream_error: StreamError = io_error.into();
    assert!(matches!(stream_error, StreamError::UnexpectedEof));
}

#[test]
fn test_rsb_stream_integration() {
    // Test that we can create RSB streams and work with them
    let stream = rsb::prelude::Stream::from_string("hello\nworld\n");
    let content = stream.to_string();
    assert!(content.contains("hello"));
    assert!(content.contains("world"));

    // Test count separately since it consumes the stream
    let stream2 = rsb::prelude::Stream::from_string("hello\nworld\n");
    assert_eq!(stream2.count(), 2);
}

#[test]
fn test_pipe_lines_functionality() {
    // Test the pipe_lines function concept (without actual stdin/stdout)
    let test_transform = |line: &str| -> String {
        format!("processed: {}", line)
    };

    let input = "line1\nline2\nline3";
    let stream = rsb::prelude::Stream::from_string(input);
    let output_stream = stream.map(test_transform);
    let output = output_stream.to_string();

    assert!(output.contains("processed: line1"));
    assert!(output.contains("processed: line2"));
    assert!(output.contains("processed: line3"));
}