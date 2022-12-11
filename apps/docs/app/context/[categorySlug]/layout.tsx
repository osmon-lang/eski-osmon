import { fetchCategoryBySlug, PageProps } from "lib/docs/getCategories";
import { Boundary } from "ui/docs/Boundary";
import { TabGroup } from "ui/docs/TabGroup";
import { Counter } from "../ContextClickCounter";

export default async function Layout({ children, params }: PageProps) {
  const category = await fetchCategoryBySlug(params.categorySlug);
  if (!category) return null;

  return (
    <Boundary labels={["Layout [Server Component]"]} animateRerendering={false}>
      <div className="space-y-9">
        <TabGroup
          path={`/context/${category.slug}`}
          items={[
            {
              text: "All",
            },
            ...category.items.map((x: { name: any; slug: any }) => ({
              text: x.name,
              slug: x.slug,
            })),
          ]}
        />
        <Counter />
        <div>{children}</div>
      </div>
    </Boundary>
  );
}
