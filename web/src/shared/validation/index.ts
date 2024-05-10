import { string } from 'zod'

const minCharactersCount = string().min(6, 'Must contain at least 6 characters')

export const emailValidation = minCharactersCount.email('Must be a valid email')
export const usernameValidation = minCharactersCount
export const passwordValidation = minCharactersCount
export const requiredString = minCharactersCount
