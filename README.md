# Overview

This is a basic multi-threaded server program in Rust. There is also a section utilizing the Rocket web framework to compare and contrast a server implementation with plain Rust vs. with Rocket.

As Rust is becoming more and more prevalent in the software industry, I decided it would be valuable to learn how to program with it. I also chose this project with the intent to learn how to program a web server from scratch. To accomplish this, I followed the tutorial in [chapter 20 of The Rust Programming Language book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

[Software Demo Video](https://youtu.be/ati1YyF3qNI)

# Development Environment

* Visual Studio Code
* Git / GitHub
* Programming language: Rust 1.71.0-nightly
* Libraries: 
    * server (ThreadPool)
    * std (fs, io, thread, net, sync)

# Useful Websites

- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [Creating a Web Service API Using Rust Rocket](https://betterprogramming.pub/creating-a-web-server-using-rust-rocket-1e4939e582df)
- [Deploy a Rust Web App With Rocket](https://www.koyeb.com/tutorials/deploy-a-rust-web-app-with-rocket)

# Future Work

- Improve error handling to not panic at every error.
- Study how to create a more robust web server in Rust and implement what I learn.
- Separate Rocket web app into different file.