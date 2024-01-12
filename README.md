# Personal Finance App

A custom-built app to help me better manage my finances.

## Background Info

I've never been the best at managing my finances. When I was young, much of my finances were looked over closely by my parents. That meant my overall financial health from an external point of view looks alright, because there are no glaring issues such as large debts or low savings.

However, their close watch also meant that I didn't develop very strong financial management skills; I have very little idea what I'm doing about my finances. I didn't want to continue down this path, so I started to take action.

The development of this app is but one step in my journey towards better financial management.

## Goals

- Keep track of how I'm keeping to my predefined budget
- Monitor overall spending trends
- Track my income levels

### Information Flow for Daily Expenditures

1. Record daily expenditures with Apple shortcuts.
2. Send information to cloud server for temporary storage.
3. Retrieve information from server on desktop app launch.
4. Update local database with information.

## Desktop app

I find that trying to build a custom app for me to better manage my finances is easier to do on desktop. Also, I generally prefer to do such data processing tasks that also involve a great deal of planning on my computer rather than on a phone.

**Tech**

Built with Tauri using Svelte because I find that I like developing with both Rust and Svelte, and want to further develop proficiency using them.

## Server

The server is mainly there because it allows me to be lazy, and I can learn more about building a web server with a personal project. The goal of the web server is to serve as a temporary storage for my data while my computer is off, then send the data to my computer at the right time.

**Tech**

Built with Go, as that is another technology that I want to develop proficiency in.
