export function filterOption(input: string | number, option: any) {
  if (!option?.key)
    return false
  const stringInput = String(input).toLowerCase()
  const label = String(option?.label).toLowerCase()
  const value = String(option?.value).toLowerCase()
  return label.includes(stringInput) || value.includes(stringInput)
}

type FilterInvalidSearchParams<T extends object, V> = {
  [K in keyof T]: Exclude<T[K], V>
}

export function filterInvalidSearchParams<T extends object>(obj: T): FilterInvalidSearchParams<T, undefined | null | ''> {
  return Object.fromEntries(Object.entries(obj)
    .filter(([, value]) => value !== undefined && value !== null && value !== '')) as any
}

export function fillOptions<T extends object, K extends keyof T>(list: T[], labelField: K, valueField: K) {
  list.forEach((item: any) => {
    item.label = item[labelField]
    item.value = item[valueField]
    if (item.children) {
      fillOptions(item.children as T[], labelField, valueField)
    }
  })

  return list
}
