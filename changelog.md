# What's new in Tmpo

## 2.4.1

### Fix

- Fix `PascalCase` transformation method.

## 2.4.0

### Feature

- Add improved values type to template meta. 
  ``` json
  {
    "key": "className", //name to use in templates
    "label": "class name", //name to show to the user
    "default": "prefix-{{name}}", //default value
    "required": false
  }
  ```

## 2.3.1

### Fix

- Use `LF` eol on windows

## 2.3.0

### Feature

- Add new `visible` flag to templates.

### Fix

- Values are now in the correct order parent -> child
- Set default path for remote template creation to `.`

## 2.2.1

### Fix

- (windows) fix init error (permission denied, os error 5)

## 2.2.0

### Feature

- Add template helper methods:
  camelCase, CONSTANT_CASE, kebab-case, lowercaser, PascalCase, snake_case, UPPERCASE

### Fix

- git remote creation
- deep target directoy

## 2.1.0

### Feature

- Folder name can now be changed with the target-directory flag or input

## 2.0.0

### Feature

- Add the possibility to share single templates via git repository with other users.

### Breaking

- Update format of configuration file.

## 1.8.1

### Fix

- Use current directory (".") as default directory for initialization. 

## 1.8.0

## Features

- Add yes option to skip optional questions at initialization

## 1.7.0

### Features

- Add no script option to prevent script execution at initialization

### Fix

- Refactoring
- Bug fixes
- Unit tests

## 1.6.0

### Features

- Sort select options alphabetically
- Fetch meta informations from the repository to propose name and description 
- Sort options alphabetically
- Start to replace master with customizable branch
- Add command to create new templates
- Many fixes to repository create command

### Fix

- Improve about texts
- Small bug fixes

## 1.5.3

### Fixes

- Initialize git before template initialization to use e.g. husky in the scripts
- Sanitize workspace if only . or ./ was passed

## 1.5.2

### Fixes

- Add skip_cleanup to travis deplyoment

## 1.5.1

## 1.5.0

### Features

- Template specific values

### Fixes

## 1.4.0

## 1.3.6

## 1.3.5

## 1.3.4

## 1.3.3

## 1.3.2

## 1.3.1

## 1.3.0

## 1.2.3

## 1.2.2

## 1.2.1

## 1.2.0

## 1.1.5

## 1.1.4

## 1.1.3

## 1.1.2

## 1.1.1

## 1.1.0

## 1.0.7

## 1.0.6

## 1.0.5

## 1.0.4

## 1.0.3

## 1.0.2

## 1.0.1

## 1.0.0

## 0.4.1

## 0.3.0

## 0.2.0

### Feature

- placeholders are now working in file and directory names

## 0.1.4

### Features

- Add username and email placeholder
- Add editorconfig
