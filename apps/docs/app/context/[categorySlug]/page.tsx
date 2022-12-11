import { fetchCategoryBySlug, PageProps } from "lib/docs/getCategories";
import { Boundary } from "ui/docs/Boundary";
import { Counter } from "../ContextClickCounter";

export default async function Page({ params }: PageProps) {
  const category = await fetchCategoryBySlug(params.categorySlug);
  if (!category) return null;

  return (
    <Boundary labels={["Page [Server Component]"]} animateRerendering={false}>
      <div className="space-y-8">
        <div className="text-xl font-medium text-gray-500">
          All {category.name}
        </div>

        <Counter />
      </div>
    </Boundary>
  );
}
