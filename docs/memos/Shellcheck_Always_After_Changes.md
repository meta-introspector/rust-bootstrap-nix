# Shellcheck: Always After Changes

## Importance of Shellcheck

`shellcheck` is a static analysis tool that gives warnings and suggestions for bash/sh shell scripts. It helps to:

*   Identify common syntax errors.
*   Detect subtle semantic problems.
*   Suggest improvements for robustness and portability.
*   Enforce coding standards and best practices.

## Policy

**It is a mandatory policy that `shellcheck` must be run on all shell scripts after any modifications are made.** This applies to new scripts, updated scripts, and any script that is part of a Change Request (CRQ) or Standard Operating Procedure (SOP).

## How to Run Shellcheck

To run `shellcheck` on a script, simply execute:

```bash
shellcheck your_script_name.sh
```

If you are working within the project's `nix develop` shell, `shellcheck` should be readily available in your `PATH`.

## Integration with CI/CD

Future Continuous Integration/Continuous Deployment (CI/CD) pipelines will include `shellcheck` as a mandatory check. Scripts that fail `shellcheck` will block merges until all issues are resolved.

## Best Practices

*   **Run frequently:** Don't wait until the end of your development cycle. Run `shellcheck` incrementally as you write and modify scripts.
*   **Address all warnings:** Treat `shellcheck` warnings as errors. Even minor suggestions can prevent future issues.
*   **Understand the output:** Familiarize yourself with `shellcheck`'s warning codes and their meanings. The tool often provides links to more detailed explanations.
