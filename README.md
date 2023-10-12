# key-kiss

--- created for ETHglobal Online Hackathon '23 ---

Welcome to 'key-kiss' (key keep it safe/secure)

A more secure way to store your private key, preventing your funds from being stolen if someone finds your 12 words paper.

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
      
![secret image](https://github.com/rtomas/key-kiss/assets/944960/c1b78d1b-5d1d-42f8-8c14-8d1c30a683e9)

2. Print the image. (example of the generated image).

    You have cols with all the posibilities in diferent order for each the 64 characters from the private key.
![269684126-dd36b2d4-19b0-4086-811d-2dfa54bee7d7](https://github.com/rtomas/key-kiss/assets/944960/d479b325-6272-443d-976f-7acd76567954)

3. Delete the png file.

4. Get your private key in metamask.

![267444635-eb064cfd-ceef-4e53-93a7-673e5625c434](https://github.com/rtomas/key-kiss/assets/944960/a77ffe8a-a546-43d6-9cc3-3eef007aa3ff)

5. Create the "secret kiss paper" like the image below from your private key
   in this link: [https://editor.p5js.org/rtomas/full/o1YL8Iw6J](https://editor.p5js.org/rtomas/full/o1YL8Iw6J)
![269683838-eb00723d-a66e-4cbe-8c17-a536fa875ef5](https://github.com/rtomas/key-kiss/assets/944960/e021044e-cce5-4f4f-911b-ef67121557f0)

6. Burn 🔥 or cut ✂ the paper with the letters.
7. Keep the "secret kiss paper" in a safe place.
8. Create a "secret kiss METAL" (soon!).

### Recovery

1. Generate the image with your personal numeric password.

    - `git clone https://github.com/rtomas/key-kiss.git`
    - `cd secret-image-cli`
    - `cargo run`
      
2. Put the "secret kiss paper" with the key-kiss characters printed or with the screen and watch you private key.
![269778499-5e8034fd-7b43-4d8b-9ba5-788af10b0a69](https://github.com/rtomas/key-kiss/assets/944960/18e57ada-6e5f-4aeb-a446-6843145e6564)


### to-do:
 + Add the version number of the cli, that was used to the create image, in the slots UI.
 + Re-enter password (compare) and hide characters.
 + Improve UX
