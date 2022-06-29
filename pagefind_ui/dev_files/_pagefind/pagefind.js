// This is a Pagefind testing stub that needs to be updated if src/output/search.js is changed

const stubbed_results = [
    {
        id: 123,
        data: {
            content: [
                `Cras mattis consectetur purus sit amet fermentum. Maecenas sed diam eget risus varius blandit sit amet non magna. Sed posuere consectetur est at lobortis. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor.`,
                `Cras mattis consectetur purus sit amet fermentum. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Curabitur blandit tempus porttitor. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras justo odio, dapibus ac facilisis in, egestas eget quam.`,
                `Nullam id dolor id nibh ultricies vehicula ut id elit. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis mollis, est non commodo luctus, nisi erat porttitor ligula, eget lacinia odio sem nec elit. Donec ullamcorper nulla non metus auctor fringilla.`
            ].join('\n'),
            url: "/cats/",
            filters: {
                color: ["Red"]
            },
            meta: {
                title: `Post about TERM`,
                image: "https://placekitten.com/800/400"
            },
            word_count: 12,
            excerpt: `some excerpt that references TERM with a nice highlighted element.`
        }
    },
    {
        id: 1234,
        data: {
            content: [
                `Cras mattis consectetur purus sit amet fermentum. Maecenas sed diam eget risus varius blandit sit amet non magna. Sed posuere consectetur est at lobortis. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor.`,
                `Cras mattis consectetur purus sit amet fermentum. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Curabitur blandit tempus porttitor. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras justo odio, dapibus ac facilisis in, egestas eget quam.`,
                `Nullam id dolor id nibh ultricies vehicula ut id elit. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis mollis, est non commodo luctus, nisi erat porttitor ligula, eget lacinia odio sem nec elit. Donec ullamcorper nulla non metus auctor fringilla.`
            ].join('\n'),
            url: "/dogs/",
            filters: {
                color: ["Blue", "Gold"]
            },
            meta: {
                title: `TERM and TERM-like things`,
                image: "https://placekitten.com/1000/500"
            },
            word_count: 15,
            excerpt: `I like TERM and also TERM...`
        }
    },
    {
        id: 12345,
        data: {
            content: [
                `Cras mattis consectetur purus sit amet fermentum. Maecenas sed diam eget risus varius blandit sit amet non magna. Sed posuere consectetur est at lobortis. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor.`,
                `Cras mattis consectetur purus sit amet fermentum. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Curabitur blandit tempus porttitor. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras justo odio, dapibus ac facilisis in, egestas eget quam.`,
                `Nullam id dolor id nibh ultricies vehicula ut id elit. Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis mollis, est non commodo luctus, nisi erat porttitor ligula, eget lacinia odio sem nec elit. Donec ullamcorper nulla non metus auctor fringilla.`
            ].join('\n'),
            url: "/llamas/",
            filters: {
                color: ["White"],
                smell: ["Sweet"]
            },
            meta: {
                title: `TERM the llama`,
                image: "https://placekitten.com/900/600",
                name: "Steve"
            },
            word_count: 100,
            excerpt: `Nullam id dolor id nibh ultricies TERM vehicula ut id elit.`
        }
    }
]


const num = (max) => Math.floor(Math.random() * max);
const stubbed_filters = (max) => {
    return {
        color: {
            Red: num(max),
            Blue: num(max),
            Green: num(max),
        },
        type: {
            Blog: num(max),
            Docs: num(max),
        }
    }
}

class Pagefind {

    async sleep(ms = 100) {
        return new Promise(r => setTimeout(r, ms));
    }

    async loadFragment(term, block) {
        await this.sleep(Math.floor(Math.random() * 2000));

        block = {
            ...block,
            excerpt: block.excerpt.replace(/TERM/g, `<mark>${term}</mark>`),
            meta: {
                ...block.meta,
                title: block.meta.title.replace(/TERM/g, term)
            }
        }

        return block;
    }

    async search(term, options) {
        await this.sleep(Math.floor(Math.random() * 2000));
        if (/y$/i.test(term)) {
            return {
                suggestion: "Hrrrrrm",
                matched: "bah",
                results: []
            }
        }

        if (options?.filters) {
            term += ` ${Object.entries(options.filters).map(([f, v]) => `[${f}:${v.join(',')}]`).join(' ')}`;
        }

        return {
            suggestion: "some suggestion",
            matched: "some match info",
            results: stubbed_results.map(r => {
                return {
                    id: r.id,
                    data: async () => await this.loadFragment(term, r.data),
                }
            }),
            filters: stubbed_filters(3),
        };
    }

    async filters() {
        await this.sleep(Math.floor(Math.random() * 2000));
        return stubbed_filters(2000);
    }
}

const pagefind = new Pagefind();

export const options = async () => { };
export const search = async (term, options) => await pagefind.search(term, options);
export const filters = async () => await pagefind.filters();
