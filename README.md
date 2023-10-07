# key-kiss

(in process)

Welcome to 'key-kiss' (key keep it safe/secure)

A more secure way to store your private key, preventing your funds from being stolen if they find your 12 words.

There are two apps in this project:

1. A cli (secret-image-cli) that creates and image since a deterministic order for characters in matrix with a personal password.

2. A web app (slots-paper-create) that lets you choose the slots of your private key character in each col.

[https://editor.p5js.org/rtomas/full/o1YL8Iw6J](https://editor.p5js.org/rtomas/full/o1YL8Iw6J)

## How it works

### Create the secret kiss paper.

1. Generate a new image with your personal password. Execute the cli.

    - `git clone https://github.com/rtomas/key-kiss.git`
    - `cd secret-image-cli`
    - `cargo run`

2. Print the image. (example of the generated image).

    You have cols with all the posibilities in diferent order for each the 64 characters from the private key.

3. Delete the png file.

4. Get your private key in metamask.

5. Create the "secret kiss paper" like the image below from your private key
   in this link: [https://editor.p5js.org/rtomas/full/o1YL8Iw6J](https://editor.p5js.org/rtomas/full/o1YL8Iw6J)

6. Burn 🔥 or cut ✂ the paper with the letters.
7. Keep the "secret kiss paper" in a safe place.
8. Create a "secret kiss METAL" (soon!).

### Recovery

1. Generate the image with your personal numeric password.

-   Clone the repo `git clone https://github.com/rtomas/key-kiss.git` and run `cargo run`

2. Put the "secret kiss paper" with the key-kiss characters printed or with the screen and watch you private key.
