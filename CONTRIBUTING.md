# Contributing to bulletty

I'm excited you're interested in contributing to **bulletty**, a TUI (Terminal User Interface) RSS/ATOM feed reader that puts data ownership back in the user's hands. As this project is under active development, your contributions are incredibly valuable. I hope you're having a good time using the tool, in the first place, and want to contribute because you believe in the project or because you think there's something that's very necessary to improve your and other people's experience. I assume you are familiar with the idea of this project and with its functionalities.

## Reporting Bugs

If you find a bug, please check the Issues to see if it has already been reported. If not, open a new issue with a clear title and description. Include as much detail as possible to help us reproduce the issue:

- A clear and concise description of the bug.
- Steps to reproduce the behavior.
- Expected behavior.
- Your operating system and terminal emulator.
- Screenshots or a video if applicable.

## Suggesting Enhancements

All ideas for new features or improvements are welcome. If you have a suggestion, please create a new topic on the [discussions page](https://github.com/CrociDB/bulletty/discussions). Describe your idea and why you think it would be a good addition to the project.

## Coding

So you want to contribute with code. That's no doubt the best way to have an influence on **bulletty**. Ideally, you would work on a previously reported issue, either by yourself or someone else.

### Working on Issues

First requirement: use the program. I've seen people wanting to contribute without using it.

Issues will only be assigned to users when enough discussion about their implementation has taken place. It's important that nobody keeps an issue assigned without making progress, as this prevents others from contributing. So, if you want to write code for an existing issue, start by discussing the issue and your proposed solution first.

I do think it's fine if you submit a PR for a bugfix you made without prior discussion, as long as you take the time to explain the **why** and the *how*. In that case, the issue won't be assigned to you until the merge is complete.

### Generative AI use

I don't want to go as far as prohibiting anyone from using AI. After all, at this point, _some AI use_ is inevitable. However, **purely vibe-coded PRs are not going to be approved**.

If you're using AI to generate code, you must make it very clear. And you'll have to own it and maintain it. I will review and ask as many questions as necessary about the code, and I reserve the right to judge whether I think the contribution is worth it or not.

Also, not properly communicating that you're using generated code in your PR is considered dishonest. If I find out, I'll have to close the PR.

## Submitting a Pull Request

1. Fork the repository and create your branch from main. Call it `feature/my-feature` or `bug/by-bug`.
2. Clone your forked repository to your local machine.
3. Implement your changes. Please ensure your code is:
    - well-written
    - formatted with `cargo fmt`
    - has unit tests when applicable (library managing, feed logic, filesystem, etc)
4. Write clear, concise commit messages.
5. Push your changes to your fork.

Open a new pull request from your branch to the main branch of **bulletty**.

Provide a clear description of the changes in your pull request. If your PR addresses an existing issue, please reference it. Images and videos are always appreciated, for a quicker understanding of what has been implemented. 

## Setting up Your Development Environment

To start contributing, you'll need to set up your local environment.

Clone the repository:

```shell
git clone https://github.com/CrociDB/bulletty.git
cd bulletty
```

Follow the instructions in the project's README.md to install dependencies and run the application.

Thank you for helping us build **bulletty**!
