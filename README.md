 

# DailySchedule

Daily schedule of OS Tutorial Summer of Code 2020

Here is the links of [rCore-Tutorial-issues](https://github.com/rcore-os/rCore-Tutorial/issues/)

# Day0

Today I applied for this project and admitted later. The tutor told me that I'd better take it step by step. To record my daily schedule, I create this repo.


The first task is to learn Rust programming language. I plan to finished the first part before 5 July 2020. Before 3 July, I have to prepare my final exams, So I will spend 2 hours per day during this period. 

# Day1

Today I planed do a revision on Rust programming language.

Luckily, the material I have learned by myself before, and I write a summary of those books. You can refer to them [Rust Summary](https://greatoyster.github.io/2020/02/01/Rust%E5%85%A5%E9%97%A8%E8%AF%AD%E6%B3%95%E5%BD%92%E7%BA%B3/#more)

And there are some small exercises in rustlings, as requested, I also intergate them in this repo folder `./rustlings`.

It seems some error while installation, I have to run  `rustup update`.

This morning, I spent about 2 hours to finish 50% of `rustlings`. 

# Day2

Today I kept learning Rust, and I found I am weak at part of error handling, I will spend more time one that.

As planed, the updated codes are contained in `rustling-exercise` folder.

# Day3

Today I updated my notes about error handling and trait, you can found it it previous link.

In fact, the polymorphism of Rust is somehow a little different from that of C-like languages.

# Day4

Toady I finished all of rustlings, after this comprehensive practice, I have better grasp of Rust.

What I need to do next is to get familiar with coming rCore-Tutorial, especially the OS related part. 

# Day5
Today I would like to experience grpc-rs. I spent the whole morning and afternoon to implemented the main functionalities of our previous class project --- RPC-based distributed file system.

I first defined the interface of using protobuf, then I generated the basic framework of the client and server. Step by step, it is smoother than our previous implementation in C language. Compared with C, Rust has less crashes in run time. In addition, the speed of developing is not so slow for the familiarity of  Rust. The new feature is that the Rust version server now support multi threads, I implemented a thread pool for the server.

In the rest of today, I learned some OS lectures,