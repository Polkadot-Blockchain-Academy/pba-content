# Plonk

A Python tutorial of the paper [PLONK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge](https://eprint.iacr.org/2019/953) it is incomplete WIP, not secure, not correct just to help people learn.

# Activity
Change the [notbook content][./plonk.ipynb] so it proves that you know the solution (3,4,5) to Pythagoras equation $x^2 + y^2 = z^2$ instead of proving the knowledge of solution to $x^3 + x + 5 = 35$ (as currently does).

## Getting Started

```bash
pip install -e .
```

See [./plonk.ipynb](./plonk.ipynb) for the tutorial.

## Testing

```bash
pip install -e .[test]
```

```bash
pytest --nbval
```
## Community

https://t.me/plonk_tutorial

## Citation

The original tutorial is from https://github.com/barryWhiteHat/plonk_tutorial
