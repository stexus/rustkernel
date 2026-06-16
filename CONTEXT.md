# Context

Glossary of the language used in this project. Definitions only — no
implementation details.

## Terms

### Test Image
A self-contained kernel binary that exercises exactly one scenario and reports a
single Verdict before powering the machine off. One Test Image boots in its own
fresh machine; Test Images never share state with one another.

### Test Harness
The orchestrator that boots each Test Image in turn, observes its serial output,
and aggregates the per-image Verdicts into one suite result. The Test Harness —
not the kernel — owns the final success/failure signal.

### Verdict
The outcome a Test Image reports for its scenario: either pass or fail. A Verdict
is communicated over serial as a Marker. Absence of a Verdict (a Test Image that
neither passes nor fails before its time runs out) is treated as a failure.

### Marker
The distinctive serial token by which a Test Image reports its Verdict: `[PASS]`
or `[FAIL]`.

### Synchronous Exception (current EL)
An exception taken at the same Exception Level that was already executing, caused
synchronously by an instruction (e.g. a data abort from an invalid memory
access). The only exception class the kernel currently handles.

### Round-trip Recovery
Successfully taking a Synchronous Exception, handling it, and resuming normal
execution at the point after the faulting instruction — as opposed to merely
reaching the exception handler. The bar a Synchronous Exception test must clear.
