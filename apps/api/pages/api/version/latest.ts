import octokit from "../../../lib/octokit";
import type { NextApiRequest, NextApiResponse } from "next";

type Data = any;

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const request = await octokit.request(
    "GET /repos/osmon-lang/osmon/releases/latest"
  );
  const latest = request.data;

  res.status(200).json(latest);
}
