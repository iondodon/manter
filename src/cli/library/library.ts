const filesOrFolders = [
    {
        name: 'file',
        regex: /^.*$/,
        next: () => [...filesOrFolders]
    }
]

const lsOptions = [
    {
        name: 'all',
        regex: /^-a$/,
        next: () => [...lsOptions, ...filesOrFolders]
    },
    {
        name: 'long',
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