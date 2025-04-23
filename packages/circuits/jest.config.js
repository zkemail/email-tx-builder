module.exports = {
    testEnvironment: 'node',
    transform: {
        '^.+\\.(ts|tsx)$': ['ts-jest', {
            tsconfig: 'tsconfig.json'
        }]
    },
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
    testMatch: ['**/tests/**/*.test.ts'],
    moduleNameMapper: {
        '^@zk-email/relayer-utils$': '<rootDir>/../../../relayer-utils/pkg',
    },
}; 