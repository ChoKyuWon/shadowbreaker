## Shadow file cracking tool
This is my toy project to use rust. For your information, this project's purpose to me is learning RUST the programming language, not make feasible Linux password cracking tool. If you are searching for some useful tool to crack the POSIX shadow file, I prefer [John the Ripper](https://www.openwall.com/john/) or [Hashcat](https://hashcat.net/hashcat/).  
However, if you're interested in my project and want to fix some errors in making some performance improvements, you're welcome at any time.

## Usage
Copy your shadow file under this directory and ```cargo run```. Follow the instructions on the screen. You must need to know your password length. If you don't know, just make assume and try it. For your information, This tool is only feasible under 3 length password. Good Luck!