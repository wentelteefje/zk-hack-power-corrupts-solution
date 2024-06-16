# puzzle-power-corrupts

An article discussing my solution can be found on my blog:
https://wentelteefje.github.io/posts/zkhack-power-corrupts-puzzle/


Puzzle description
==================


    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\


Bob has invented a new pairing-friendly elliptic curve, which he wanted to use with Groth16.
For that purpose, Bob has performed a trusted setup, which resulted in an SRS containting
a secret $\tau$ raised to high powers multiplied by a specific generator in both source groups.
The exact parameters of the curve and part of the output of the setup are described in the
document linked below.

Alice wants to recover $\tau$ and she noticed a few interesting details about the curve and
the setup. Specifically, she noticed that the sum $d$ of the highest power $d_1$ of $\tau$ in
$\mathbb{G}_1$ portion of the SRS, meaning the SRS contains an element of the form
$\tau^{d_1} G_1$ where $G_1$ is a generator of $\mathbb{G}_1$, and the highest power $d_2$
of $\tau$ in $\mathbb{G}_2$  divides $q-1$, where $q$ is the order of the groups.

Additionally, she managed to perform a social engineering attack on Bob and extract the
following information: if you express $\tau$ as $\tau = 2^{k_0 + k_1((q-1/d))} \mod r$,
where $r$ is the order of the scalar field, $k_0$ is 51 bits and its fifteen most
significant bits are 10111101110 (15854 in decimal). That is A < k0 < B where
A = 1089478584172543 and B = 1089547303649280.

Alice then remembered the Cheon attack...

NOTE: for exponentiating $F_r$ elements, use the `pow_sp` and `pow_sp2` functions in
`utils.rs`.

The parameters of the curve and the setup are available at
https://gist.github.com/kobigurk/352036cee6cb8e44ddf0e231ee9c3f9b
