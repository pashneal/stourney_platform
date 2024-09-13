---
title: Quick start guide
intro: Need to get started quickly with Stourney? This guide will help you get up and running in no time.
section: Getting Started
show_video_tour: false
show_quick_links: false
order: 1
permalink: /
---

## Installation

To get started with Stourney, you need to install the Stourney CLI. The CLI is a command-line tool that helps you create, manage, and deploy your Stourney bots.

You'll need:

**Rust >= 1.51** \
**Python >= 3.8**

To install the CLI, run the following command:

```bash
cargo install stourney
```

## Running your first project

To get started with a new Stourney project, issue this command to your terminal:

```bash
stourney new my_project
```

This will create a new Stourney project in the `my_project` directory which contains the source code for your bot as well as scaffolding for getting your bot running.

Once the project is created, you can change the tournament settings by issuing the following command:

```bash
stourney config edit
```

Once you've decided on the competitors and tournament settings, it is now time to watch the bots compete against each other! To run the tournament, type the following command:

```bash
stourney run
```

This will start the tournament and display the results in the terminal, along with 
any output from the logs of the bots or errors.



