import { fetchCategoryBySlug, type PageProps } from "lib/docs/getCategories";
import { ClickCounter } from "ui/docs/ClickCounter";
import { TabGroup } from "ui/docs/TabGroup";

export default async function Layout({ children, params }: PageProps) {
  const category = await fetchCategoryBySlug(params.categorySlug);
  if (!category) return null;

  return (
    <div className="space-y-9">
      <div className="flex justify-between">
        <TabGroup
          path={`/head/${category.slug}`}
          items={[
            {
              text: "All",
            },
            ...category.items.map((x) => ({
              text: x.name,
              slug: x.slug,
            })),
          ]}
        />
      </div>

      <div>{children}</div>
    </div>
  );
}
