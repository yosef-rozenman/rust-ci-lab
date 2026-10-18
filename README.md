# CI Runner Onboarding Lab

A small Rust project for Mission 1 on page 2 of the CI Runner Handbook.

## 1. Install Rust

Install Rust from https://rustup.rs/ if needed. Open a new terminal in this folder.

## 2. Run the project

```bash
rustc --version
cargo --version
cargo run -- 2500 2500
cargo test --locked
```

Expected program output: `Total: 5000 cents`. All five tests should pass.

## 3. Find and fix a bug (Mission 1)

Open `src/lib.rs`. Find:

```rust
let shipping = if subtotal >= 5000 { 0 } else { 500 };
```

Change `>=` to `>` and run `cargo test --locked` again.
The test `exact_threshold_has_free_shipping` should fail: the wrong code charges 500 cents shipping when the subtotal is exactly 5000 cents. Change `>` back to `>=` and run the tests again. Keep the test!

## 4. Put it on GitHub (Mission 2)

Create an empty GitHub repository named `ci-runner-onboarding-lab`. From this folder:

```bash
git init
git add .
git commit -m "Add Rust CI onboarding lab"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/ci-runner-onboarding-lab.git
git push -u origin main
```

Open **Actions** -> **Rust baseline** -> **Run workflow**. Select `linux` and run it. Then select `windows` and run it. Save both run URLs.

## 5. Next step (Mission 3)

Ask your mentor for your repository's assigned Incredibuild Linux and Windows runner labels. Do not guess labels: GitHub Actions cannot create runners just by using their names. Compare runs using the same code commit.

## Mission 1 checklist

- [ ] `rustc --version` works
- [ ] `cargo --version` works
- [ ] `cargo run -- 2500 2500` prints 5000 cents
- [ ] `cargo test --locked` passes
- [ ] Changing `>=` to `>` makes the exact-threshold test fail
- [ ] Restoring `>=` makes the tests pass again
