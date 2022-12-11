import octokit from "../../../lib/octokit";
import type { NextApiRequest, NextApiResponse } from "next";

type Data = any;

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  const { version } = req.query;
  const request = await octokit
    .request(`GET /repos/osmon-lang/osmon/releases/tags/${version}`)
    .catch(() => {});
  const tag = request
    ? request.data
    : {
        status: 404,
        message: "Not found",
      };
  res.status(200).json(tag);
}
