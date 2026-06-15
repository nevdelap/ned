# Agent Guidance

These instructions are for coding agents (e.g., GitHub Copilot, Cline, Kilo Code) working in this repository.

- Do not abbreviate unnecessarily: prefer `component` over `comp`, `filename`
  over `fname`.
- Avoid single-letter variable names unless explicitly requested.
- Use descriptive, clear names for variables, functions, files, and tests.
- Preserve existing code style and naming conventions; make minimal, focused
  changes.
- When updating code, run tests to verify behavior before and after changes.
- When removing code, don't leave comments about the removed code.
- Never add comments explaining what the agent is doing because what the agent
  is doing isn't appropriate for commiting to source control.
- Add brief, helpful comments only where intent is non-obvious; avoid noisy
  commentary.
- Follow platform nuances (e.g., symlink handling) using runtime checks rather
  than assumptions.
- When in doubt, choose clarity over brevity.
- When making changes update the `CHANGELOG.md` section for the version as it is
  currently set.
- Do not included in the `CHANGELOG.md` any dates for versions.
- Include in the `CHANGELOG.md` only the changes that are visible/relevant to an
  end user.
- Specify which changes in the `CHANGELOG.md` are breaking changes.
