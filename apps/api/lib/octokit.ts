import { Octokit, App } from "octokit";

const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });

export default octokit;
