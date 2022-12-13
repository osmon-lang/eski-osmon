import { InferGetStaticPropsType } from 'next'
import { logs } from '../releases'
// Layout
import PageLayout from 'components/layout/page'
import { getHashtagTweets, TwitterRes } from 'lib/twitter'

// Sections
import Hero from 'components/sections/hero'
import AboutSection from 'components/sections/about'
import CharactersSection from 'components/sections/characters'
import DataColumns from 'components/sections/data-columns'

const HomePage = ({
  tweets
}: InferGetStaticPropsType<typeof getStaticProps>) => {
  return (
    <PageLayout>
      <Hero />
      <AboutSection />
      <CharactersSection />
      <DataColumns tweets={tweets} releases={logs} />
    </PageLayout>
  )
}

export const getStaticProps = async () => {
  let tweets: TwitterRes | null
  try {
    tweets = await getHashtagTweets()
  } catch (error) {
    // @ts-ignore
    console.warn(error.message)
    tweets = null
  }

  return {
    props: {
      tweets
    },
    revalidate: 1
  }
}

export default HomePage
