# Foreclosure Scaper Context Engineering 

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.  It contains rules for Rust development and deployment 


## Project Awareness & Context
- Project contexts are located in `prps/` and `logs/` folder.
- At the start of a new conversation, read`/prps/projectplan.md`to review project's architecture, style, and constraints.
- At the start of a new conversation, review sessions log files in logs to understand project status and issues.

## Tasks 
- Use the `tasks.md` file in the `prps/` to track the status of all the tasks that need to be done
- Add new tasks to the tasks.md file 
- Do not work on tasks in the tasks.md that have already been completed.  Do not repeat these tasks.
- **Mark completed tasks in `tasks.md`** immediately after finishing them.
- Add new sub-tasks or TODOs discovered during development to `tasks.md` under a “Discovered During Work” section.

## Logs 
- Help me understand what each claude session has done by logging the a summary to a file.
- Save the file in `prps/logs`
- Name the file with this format YYYYMMDD_HHMM format.
- Add date and time as part of the content.
- Summarize each session with heading and bullet points.

## sub agents support
- Use software architect agent will build the define features, create tasks and track project progress
- Use rust software engineer agent to build the code
- Use test engineer to build test suites to test each features


## Code Structure & Modularity
- **Never create a file longer than 500 lines of code.** If a file approaches this limit, refactor by splitting it into modules or helper files.
- **Organize code into clearly separated modules**, grouped by feature or responsibility.
- Never hardcode sensitive information - Always use .env files for API keys and configuration

## Testing & Reliability
- **After updating any logic**, check whether existing unit tests need to be updated. If so, do it.
- **Tests should live in a `/tests` folder** mirroring the main app structure.
  - Include at least:
    - 1 test for expected use
    - 1 edge case
    - 1 failure case

### 📎 Modification Guideline
- When modifying code, always ... tbd 

### 📎 Style & Conventions
- **To be determine

### 📚 Documentation & Explainability
- **Update `README.md`** when new features are added, dependencies change, or setup steps are modified.
- **Comment non-obvious code** and ensure everything is understandable to a mid-level developer.
- When writing complex logic, **add an inline `# Reason:` comment** explaining the why, not just the what.

### 🧠 AI Behavior Rules
- **Never assume missing context. Ask questions if uncertain.**
- **Never hallucinate libraries or functions** – only use known, verified Python packages.
- **Always confirm file paths and module names** exist before referencing them in code or tests.
- **Never delete or overwrite existing code** unless explicitly instructed to or if part of a task from `task.md`.
