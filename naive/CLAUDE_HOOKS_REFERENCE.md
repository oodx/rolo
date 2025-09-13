# Claude CLI Hooks Reference

## Quick Hook Setup

### 1. Create Hook Script
```bash
# Create the hook script
cat > /tmp/claude-tool-logger.sh << 'EOF'
#!/bin/bash

# Read JSON data from stdin
JSON_INPUT=$(cat)

# Parse the JSON to extract useful info
TOOL_NAME=$(echo "$JSON_INPUT" | jq -r '.tool_name // "unknown"')
SESSION_ID=$(echo "$JSON_INPUT" | jq -r '.session_id // "unknown"')

# Log the tool usage
echo "$(date): Tool: $TOOL_NAME" >> /tmp/claude-tool-usage.log
echo "$JSON_INPUT" >> /tmp/claude-tool-usage.log
echo "---" >> /tmp/claude-tool-usage.log

# For Bash commands, extract the actual command
if [[ "$TOOL_NAME" == "Bash" ]]; then
    COMMAND=$(echo "$JSON_INPUT" | jq -r '.tool_input.command // "no command"')
    echo "$(date): Bash Command: $COMMAND" >> /tmp/bash-commands.log
fi
EOF

chmod +x /tmp/claude-tool-logger.sh
```

### 2. Configure Claude CLI Settings
Edit `/home/xnull/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "printf '\\n\\e[1;36m%s\\n[%s]\\e[m \\e[1;32m\\n[%s][%s]\\e[m' \"$(printf '%.0s-' {1..60})\" \"$(pwd)\" \"$(whoami)\" \"$(date +%H:%M)\""
  },
  "model": "opusplan",
  "hooks": {
    "PreToolUse": [{
      "hooks": [{
        "type": "command",
        "command": "/tmp/claude-tool-logger.sh"
      }]
    }]
  }
}
```

### 3. Test Hook
```bash
# Check if it's working
tail -f /tmp/claude-tool-usage.log &
# Use Claude CLI and watch the log update
```

## Hook Types Available
- **PreToolUse** - Before tool execution
- **PostToolUse** - After tool execution
- **SessionStart** - When session begins
- **SessionEnd** - When session ends

## JSON Data Structure
The hook receives JSON via stdin with these fields:
```json
{
  "tool_name": "Bash",
  "session_id": "fb8eedbf-1f8c-4978-9397-c2a8befa0875",
  "tool_input": {
    "command": "ls -la",
    "description": "List files"
  }
}
```

## Environment Variables Available
- `CLAUDE_PROJECT_DIR` - Current project directory

## Cleanup/Remove Hooks
To remove hooks, edit settings.json and remove the "hooks" section:
```json
{
  "statusLine": { ... },
  "model": "opusplan"
}
```

## Use Cases
- **Development logging** - Track what commands Claude runs
- **Security auditing** - Monitor file operations
- **Context monitoring** - Watch token usage (if extracting from session JSONL)
- **Auto-commit triggers** - Git commit on file changes
- **Notification system** - Alert on certain tool usage

## Previous Discovery Notes
From SESSION_01: We found that hooks receive JSON via stdin (NOT environment variables as docs suggested). The `cache_read_input_tokens` field in session JSONL files gives real-time active context matching Claude's `/context` command with 99.5% accuracy.

## Warning
Hook scripts that error or take too long can slow down Claude CLI. Keep them fast and robust!