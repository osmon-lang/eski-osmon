# Turborepo Workspace

Turborepo is used in order to maintain all web applications in a single repository.

## What's inside?

This turborepo uses [pnpm](https://pnpm.io) as a package manager. It includes the following packages/apps:

### Apps and Packages

- `docs`: documentation website to keep lang docs
- `web`: website to show the project
- `api`: api server to serve the project
- `ui`: a stub React component library shared by `web`, `api` and `docs` applications
- `eslint-config-custom`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This turborepo has some additional tools already setup:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Build

To build all apps and packages, run the following command:

```
cd osmon
pnpm run build
```

### Develop

To develop all apps and packages, run the following command:

```
cd osmon
pnpm run dev
```

[//]: # (### Remote Caching)

[//]: # ()
[//]: # (Turborepo can use a technique known as [Remote Caching]&#40;https://turbo.build/repo/docs/core-concepts/remote-caching&#41; to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.)

[//]: # ()
[//]: # (By default, Turborepo will cache locally. To enable Remote Caching you will need an account with Vercel. If you don't have an account you can [create one]&#40;https://vercel.com/signup&#41;, then enter the following commands:)

[//]: # ()
[//]: # (```)

[//]: # (cd my-turborepo)

[//]: # (pnpm dlx turbo login)

[//]: # (```)

[//]: # ()
[//]: # (This will authenticate the Turborepo CLI with your [Vercel account]&#40;https://vercel.com/docs/concepts/personal-accounts/overview&#41;.)

[//]: # ()
[//]: # (Next, you can link your Turborepo to your Remote Cache by running the following command from the root of your turborepo:)

[//]: # ()
[//]: # (```)

[//]: # (pnpm dlx turbo link)

[//]: # (```)

## Useful Links

Learn more about the power of Turborepo:

- [Pipelines](https://turbo.build/repo/docs/core-concepts/monorepos/running-tasks)
- [Caching](https://turbo.build/repo/docs/core-concepts/caching)
- [Remote Caching](https://turbo.build/repo/docs/core-concepts/remote-caching)
- [Filtering](https://turbo.build/repo/docs/core-concepts/monorepos/filtering)
- [Configuration Options](https://turbo.build/repo/docs/reference/configuration)
- [CLI Usage](https://turbo.build/repo/docs/reference/command-line-reference)
