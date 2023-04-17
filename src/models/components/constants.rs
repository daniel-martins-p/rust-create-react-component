pub const EXPORT_INDEX_FILE_NAME: &str = "index.tsx";
pub const COMPONENT_TEMPLATE: &str = r#"function {component_name}() {
  return (
    <div data-testid="{component_name}">{component_name}Component</div>
  );
}

export default {component_name};
"#;
pub const COMPONENT_TEST_TEMPLATE: &str = r#"import {component_name} from "./{component_name}";
import { render } from "@testing-library/react";

describe("{component_name}", () => {
  it("should render", () => {
    const { getByTestId } = render(<{component_name} />);
    expect(getByTestId("{component_name}")).toBeInTheDocument();
  });
});
"#;
pub const COMPONENT_EXPORT_INDEX_TEMPLATE: &str = r#"export { default } from './{component_name}';"#;