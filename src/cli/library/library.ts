const filesOrFolders = [
    {
        name: 'file or foler',
        regex: /^.*$/,
        next: () => [...filesOrFolders]
    }
]

const lsOptions = [
    {
        name: '-a',
        regex: /^-a$/,
        next: () => [...lsOptions, ...filesOrFolders]
    },
    {
        name: '-l',
        regex: /^-l$/,
        next: () => [...lsOptions, ...filesOrFolders]
    }
]

const clis = [
    {
        name: 'ls',
        regex: /^ls$/,
        next: () => [...lsOptions, ...filesOrFolders]
    }
]

export default clis