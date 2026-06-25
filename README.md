# Chaum-Pedersen Proof (Rust Toy Implementation)

This project implements a toy version of the **Chaum-Pedersen protocol**: a zero-knowledge proof that two public values share the same hidden exponent.

It proves knowledge of `x` such that:

- `y1 = alpha^x mod p`
- `y2 = beta^x mod p`

without revealing `x`.

## What the Scheme Proves

Given public parameters `(alpha, beta, p, q)` and public values `(y1, y2)`, a prover shows:

- `log_alpha(y1) = log_beta(y2) = x`

This is an equality-of-discrete-logs proof.

## What Chaum-Pedersen Is (Intuition)

Chaum-Pedersen is a **zero-knowledge proof of equality of exponents**.

- You have two public bases (`alpha`, `beta`).
- You publish two values (`y1`, `y2`).
- You want to prove both were made with the same secret exponent `x`.

In other words, you prove this statement without leaking `x`:

- `y1 = alpha^x mod p`
- `y2 = beta^x mod p`

Why this is useful:

- It proves consistency of secret data across two equations.
- It is used inside larger protocols (credentials, signatures, ZK systems).
- The verifier learns only that the statement is true, not the secret itself.

## Interactive Flow (Prover vs Verifier)

1. Prover picks random `k` and sends commitments:

- `r1 = alpha^k mod p`
- `r2 = beta^k mod p`

2. Verifier sends random challenge `c`.
3. Prover answers with response `s`.

- In this repo's variant: `s = k - c*x mod q`

4. Verifier checks both equations:

- `r1 ?= alpha^s * y1^c mod p`
- `r2 ?= beta^s * y2^c mod p`

If both pass, verifier accepts.

Security intuition:

- Completeness: an honest prover always passes.
- Soundness: a cheater who does not know `x` cannot answer random `c` reliably.
- Zero-knowledge (interactive form): verifier gets no usable information to recover `x`.

## Protocol Math (Variant Used in This Code)

This code uses the response form:

- `s = k - c*x mod q`

with commitments:

- `r1 = alpha^k mod p`
- `r2 = beta^k mod p`

Verifier checks:

- `r1 ?= alpha^s * y1^c mod p`
- `r2 ?= beta^s * y2^c mod p`

Why this works:

- `alpha^s * y1^c = alpha^(k - cx) * (alpha^x)^c = alpha^k = r1`
- `beta^s * y2^c = beta^(k - cx) * (beta^x)^c = beta^k = r2`

So if both equations hold, the prover is consistent with one shared secret `x`.

## Mapping Math to Code

Implementation is in `src/lib.rs`.

- `exponentiate(n, exponent, modulus)`
  - Computes modular exponentiation `n^exponent mod modulus` using `modpow`.

- `solve(k, challenge, secret, modulus)`
  - Computes `s = k - c*x mod q`.
  - Handles underflow for unsigned integers by doing:
    - direct subtraction when `k >= c*x`
    - otherwise `q - ((c*x - k) mod q)`

- `verify(...)`
  - Checks both Chaum-Pedersen equations:
    - `r1 == (alpha^s * y1^c) mod p`
    - `r2 == (beta^s * y2^c) mod p`
  - Returns `true` only if both are true.

- `generate_random_below(bound)`
  - Samples random `BigUint` values below `bound` (used for toy random tests).

## Toy Example in Tests

The fixed test uses:

- `alpha = 4`, `beta = 9`, `p = 23`, `q = 11`
- secret `x = 6`, nonce `k = 7`, challenge `c = 4`

Then:

- `y1 = 4^6 mod 23 = 2`
- `y2 = 9^6 mod 23 = 3`
- `r1 = 4^7 mod 23 = 8`
- `r2 = 9^7 mod 23 = 4`
- `s = 7 - 4*6 mod 11 = 5`

Verifier checks:

- `4^5 * 2^4 mod 23 = 8 = r1`
- `9^5 * 3^4 mod 23 = 4 = r2`

So verification succeeds.

The test also tries a fake secret (`x = 7`) and verification fails, as expected.

## Running Tests

```bash
cargo test
```

## Important Notes

This is a learning implementation, not production cryptography.

- Parameters are tiny and insecure.
- No subgroup membership/cofactor checks are enforced.
- Random sampling and protocol flow are simplified.
- For real systems, use audited libraries and standardized groups/curves.
