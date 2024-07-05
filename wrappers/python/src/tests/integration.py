from pagefind_python.index import PagefindIndex, IndexConfig


async def main():
    config = IndexConfig(
        root_selector="main", logfile="index.log", output_path="./output", verbose=True
    )
    async with PagefindIndex(config=config) as index:
        await index.add_directory("./public")
        new_file = await index.add_html_file(
            content=(
                "<html>"
                "  <body>"
                "    <main>"
                "      <h1>Example HTML</h1>"
                "      <p>This is an example HTML page.</p>"
                "    </main>"
                "  </body>"
                "</html>"
            ),
            url="https://example.com",
            source_path="other/example.html",
        )
        print(f"new_file={new_file}")
        new_record = await index.add_custom_record(
            url="/elephants/",
            content="Some testing content regarding elephants",
            language="en",
            meta={"title": "Elephants"},
        )
        print(f"new_record={new_record}")

        new_dir = await index.add_directory("./public")
        print(f"new_dir={new_dir}")

        files = await index.get_files()
        for f in files:
            print(f"files= {len(f['content']):10}B {f['path']}")


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
