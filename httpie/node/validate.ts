import { string, array } from 'yup';

export const validateUrl = (url: string) => {
  return string().url().validateSync(url);
}

export const validateBody = (body: string[]) => {
  return array().of(string().matches(/\S+=\S+/)).validateSync(body);
}


