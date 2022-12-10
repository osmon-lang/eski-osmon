import type { NextApiRequest, NextApiResponse } from "next";

type Data = {
  ip: string;
  city: string | null;
  isCold: boolean;
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  // Get IP address from request headers
  const parsedCity = decodeURIComponent(
    <string>req.headers["x-vercel-ip-city"]
  );

  // Parse the city from IP Address
  const city = parsedCity === "null" ? null : parsedCity;

  // Get IP address from request headers
  // @ts-ignore
  const ip = (req.headers["x-forwarded-for"] ?? "127.0.0.1").split(",")[0];

  // Return the data
  res.status(200).json({ ip, city, isCold: true });
}
