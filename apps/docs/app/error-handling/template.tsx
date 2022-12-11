import { Boundary } from "ui/docs/Boundary";

export default function Page({ children }: { children: React.ReactNode }) {
  return <Boundary>{children}</Boundary>;
}
