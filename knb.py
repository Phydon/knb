#!/usr/bin/env python

from rich.console import Console  # V13.5.3
from rich.markdown import Markdown
from rich.table import Table
from rich.tree import Tree

# rich - colorful output
console = Console(highlight=False)

PATH_TO_KNOWLEDGE_BASE: str = "7z.md"
# PATH_TO_KNOWLEDGE_BASE: str = "test.md"


def main() -> None:
    content = read_file(PATH_TO_KNOWLEDGE_BASE)
    console.print(Markdown(content))


# def table() -> None:
#     table = Table(
#         title="Test",
#         title_style="bold",
#         show_lines=True,
#     )

#     table.add_column("Test1", justify="left", style="white")
#     table.add_column("Test1", justify="right", style="dim italic")
#     table.add_column("Test1", justify="right", style=" dim red")
#     table.add_column("Test1", justify="right", style="dim blue")
#     table.add_column("Test1", justify="right", style="blue")
#     table.add_column("Test1", justify="right", style="blue")
#     table.add_column("Test1", justify="right", style="red")
#     table.add_column("Test1", justify="right", style="green")

#     console.print(table)


# def tree() -> None:
#     tree = Tree("\n[b]Test[/b]")
#     tree.add("[i]Test1[/i]: [blue]wasd[/blue]")
#     tree.add("[i]Test2[/i]: [green]wasd[/green]")
#     tree.add("[i]Test3[/i]: [red]wasd[/red]")
#     console.print(tree)


# def markdown() -> None:
#     console.print(Markdown("# Test Heading"))
#     console.print(Markdown("## Test Heading 2"))
#     console.print(Markdown("### Test Heading 3"))
#     console.print(Markdown("#### Test Heading 4"))
#     console.print(Markdown("__Test__"))
#     console.print(Markdown("_Test_"))


def read_file(path: str) -> str:
    with open(path, "r") as f:
        content = f.read()

    return content


if __name__ == "__main__":
    main()
