import octokit from "../../../lib/octokit";
import type { NextApiRequest, NextApiResponse } from "next";

type Data = string[];

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const request = await octokit.request("GET /repos/osmon-lang/osmon/releases");
  const versions = request.data.map(
    (version: { tag_name: string }) => version.tag_name
  );

  res.status(200).json(versions);
}
