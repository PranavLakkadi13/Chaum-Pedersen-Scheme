# Chaum-Pedersen Scheme and Project Overview

This project is a educational implementation of the **Chaum-Pedersen protocol** and related zero-knowledge ideas. The goal is to understand how a prover can convince a verifier that they know a secret value without revealing the secret itself.

In simple terms:
- A **Chaum-Pedersen proof** shows that two public values were created using the same hidden exponent.
- A **commitment** hides a secret value now and lets you reveal it later in a way the verifier can check.
- The code in this repository demonstrates the math behind that idea using small toy values so the steps are easy to follow.

---

## What is Chaum-Pedersen?

Chaum-Pedersen is a **zero-knowledge proof of equality of discrete logarithms (exponents)**. Given two public bases $\alpha$ and $\beta$, and two public values $y_1$ and $y_2$, the prover demonstrates knowledge of a secret $x$ such that:

- $y_1 = \alpha^x \pmod p$
- $y_2 = \beta^x \pmod p$

without revealing $x$.

Why this is useful:
- It proves consistency of secret data across two equations.
- It is used inside larger protocols (like credentials, blind signatures, and voting systems).
- The verifier learns only that the statement is true, but does not learn the secret exponent itself.

> [!NOTE]
> A commitment scheme is like putting a secret inside a locked envelope. It is **hiding** (the verifier cannot see the secret yet) and **binding** (once you commit, you cannot change the secret later). Pedersen commitments use modular exponentiation and hidden randomness similarly to this scheme.

---

## Parameter Roles

The symbols in the math and the code have the following roles:

| Value | Role | Where it lives |
| :--- | :--- | :--- |
| `g`, `h` (or `alpha`, `beta`) | Generators | Modulo $p$ (range $[1, p - 1]$) |
| `y1`, `y2` | Public keys | Modulo $p$ (range $[1, p - 1]$) |
| `x` | Private key / secret exponent | Modulo $q$ (range $[1, q - 1]$) |
| `k` | Random nonce / blinding factor | Modulo $q$ (range $[1, q - 1]$) |
| `c` | Verifier challenge | Modulo $q$ (range $[1, q - 1]$) |
| `s` | ZKP response | Modulo $q$ (range $[1, q - 1]$) |

---

## Interactive Flow & Protocol Math

The variant of the protocol implemented in this code works as follows:

1. **Commitment (Prover):** Prover picks a random $k \in [1, q-1]$ and sends:
   - $r_1 = \alpha^k \pmod p$
   - $r_2 = \beta^k \pmod p$

2. **Challenge (Verifier):** Verifier sends a random challenge $c \in [1, q-1]$.

3. **Response (Prover):** Prover computes and sends:
   - $s = k - c \cdot x \pmod q$

4. **Verification (Verifier):** Verifier accepts if and only if both equations hold:
   - $r_1 \stackrel{?}{=} \alpha^s \cdot y_1^c \pmod p$
   - $r_2 \stackrel{?}{=} \beta^s \cdot y_2^c \pmod p$

### Mathematical Proof of Correctness
$$\alpha^s \cdot y_1^c \equiv \alpha^{k - cx} \cdot (\alpha^x)^c \equiv \alpha^{k - cx + cx} \equiv \alpha^k \equiv r_1 \pmod p$$
$$\beta^s \cdot y_2^c \equiv \beta^{k - cx} \cdot (\beta^x)^c \equiv \beta^{k - cx + cx} \equiv \beta^k \equiv r_2 \pmod p$$

---

## NUMS Idea for the Second Generator

In a real system, the second generator $h$ should be generated so nobody knows the discrete logarithm relationship between $g$ and $h$. If the person who sets up the parameters knows $x$ such that $h = g^x$, they can break the binding property of the commitment.

One common approach is to use a **Nothing-Up-My-Sleeve (NUMS)** style generation method:
1. Pick a random value $y$ in the range $[2, p - 1]$.
2. Project it into the subgroup of order $q$ by computing $h = y^{(p - 1) / q} \pmod p$.
3. If $h = 1$, discard it and try again.

This gives a valid second generator candidate without intentionally choosing a value with a known secret relation.

---

## Mapping Math to Code

The implementation is located in [lib.rs](file:///Users/pranavlakkadi/Codes/ZKP-rust-udemy/src/lib.rs).

- **`exponentiate(n, exponent, modulus)`**: Computes modular exponentiation $n^{\text{exponent}} \pmod{\text{modulus}}$.
- **`solve(k, challenge, secret, modulus)`**: Computes $s = k - c \cdot x \pmod q$, handling underflow for unsigned integers.
- **`verify(...)`**: Evaluates the two verification checks and returns `true` if both pass.
- **`generate_random_below(bound)`**: Samples random `BigUint` values below the specified bound.

---

## Toy Example in Tests

The unit test verifies the protocol correctness using the following parameters:
- $\alpha = 4$, $\beta = 9$, $p = 23$, $q = 11$
- secret $x = 6$, nonce $k = 7$, challenge $c = 4$

### Computations:
- $y_1 = 4^6 \pmod{23} = 2$
- $y_2 = 9^6 \pmod{23} = 3$
- $r_1 = 4^7 \pmod{23} = 8$
- $r_2 = 9^7 \pmod{23} = 4$
- $s = 7 - (4 \times 6) \pmod{11} = 5$

### Verification:
- $4^5 \cdot 2^4 \pmod{23} = 8 = r_1$
- $9^5 \cdot 3^4 \pmod{23} = 4 = r_2$

Verification succeeds. The tests also verify that a fake secret $x_{\text{fake}} = 7$ fails validation.

---

## Running Tests

```bash
cargo test
```

> [!WARNING]
> This is a learning implementation, not production cryptography. The parameters are small/insecure, subgroup checks are not enforced, and random sampling is simplified. Use audited libraries for real systems.
