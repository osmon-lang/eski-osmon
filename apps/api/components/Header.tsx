import Head from "next/head";

export default function Header() {
  return (
    <Head>
      <title>Osmon Registry</title>
      <meta charSet="utf-8" />
      <link rel="icon" href="/favicon.ico" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta
        name="description"
        content="Backend Serverless API made for Osmon"
      />
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:site" content="@uwussimo" />
      <meta name="twitter:creator" content="@uwussimo" />
      <meta name="twitter:title" content="Osmon API" />
      <meta
        name="twitter:description"
        content="Backend Serverless API made for Osmon"
      />
      <meta name="twitter:image" content="/card.png" />
      <meta name="twitter:image:alt" content="Osmon Lang Open Graph" />
    </Head>
  );
}
