import useSWR from "swr";
import type { NextPage } from "next";
import Header from "../components/Header";
import Card from "../components/Card";
import Footer from "../components/Footer";

// @ts-ignore
const fetcher = (...args) => fetch(...args).then((res) => res.json());

const Home: NextPage = () => {
  const { data, error } = useSWR("/api/info", fetcher);

  if (error) {
    return <div>Oops, there is something wrong with server...</div>;
  }

  if (!data) {
    return (
      <html lang="en">
        <Header />
        <body>
          <div style={{ height: "100%" }}>
            <Card />
          </div>
          <Footer />
        </body>
      </html>
    );
  }

  const { ip, city, isCold } = data;

  return (
    <html lang="en">
      <Header />
      <body>
        <div style={{ height: "100%" }}>
          <Card />

          <main>
            <h1>
              <span>Osmon Registry</span>
            </h1>

            <div className="info">
              <div className="block">
                <div className="contents">
                  <span>Your city</span>
                  <strong
                    title={
                      city === null
                        ? "GeoIP information could not be derived from your IP"
                        : undefined
                    }
                    // @ts-ignore
                    className={city === null ? "na" : null}
                  >
                    {city === null ? "N/A" : city}
                  </strong>
                </div>
              </div>

              <div className="block">
                <div className="contents">
                  <span>Your IP address</span>
                  <strong>{ip}</strong>
                </div>
              </div>
            </div>
          </main>
          <div className="debug">
            Generated at {new Date().toISOString()} ({isCold ? "cold" : "hot"})
          </div>
        </div>

        <Footer />
      </body>
    </html>
  );
};

export default Home;
