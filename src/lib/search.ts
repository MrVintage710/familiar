import fuzzysearch from "fuzzysearch-ts"

export type Searchable = {
  tags: string[],
  name: string
}

export default function search(items : Searchable[], searchText : string, tags : string[] = []) : Searchable[] {
  return items
    .filter(serchable => tags.length == 0 ? true : tags.some(tag => serchable.tags.includes(tag)) )
    .filter(searchable => fuzzysearch(searchText, searchable.name))
}