import { Octokit, App } from "octokit";

const octokit: Octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });

export default octokit;
