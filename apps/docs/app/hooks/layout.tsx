import { fetchCategories } from "lib/docs/getCategories";
import { Boundary } from "ui/docs/Boundary";
import { ClickCounter } from "ui/docs/ClickCounter";
import HooksClient from "ui/docs/HooksClient";
import HooksServer from "ui/docs/HooksServer";
import { TabGroup } from "ui/docs/TabGroup";
import React from "react";

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  const categories = await fetchCategories();
  if (!categories) return null;
  return (
    <div className="space-y-9">
      <div className="flex justify-between">
        <TabGroup
          path="/hooks"
          items={[
            {
              text: "Home",
            },
            ...categories.map((x) => ({
              text: x.name,
              slug: x.slug,
            })),
          ]}
        />

        <div className="self-start">
          <ClickCounter />
        </div>
      </div>

      <Boundary labels={["Client Component Hooks"]}>
        <HooksClient />
      </Boundary>
      <Boundary labels={["Server Component Hooks"]}>
        <HooksServer />
      </Boundary>

      <div>{children}</div>
    </div>
  );
}
