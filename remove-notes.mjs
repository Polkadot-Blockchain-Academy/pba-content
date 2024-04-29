import fs from 'fs';
import path from 'path';

async function removeMultilineSequences(directoryPath) {
    try {
        const files = await fs.promises.readdir(directoryPath);

        for (const file of files) {
            if ((await fs.promises.lstat(path.join(directoryPath, "/", file))).isDirectory()) {
                removeMultilineSequences(path.join(directoryPath, "/", file))
            } else {

                const filePath = path.join(directoryPath, file);
                const fileContent = await fs.promises.readFile(filePath, 'utf-8');
                const modifiedContent = removeSequences(fileContent);

                if (modifiedContent !== fileContent) {
                    await fs.promises.writeFile(filePath, modifiedContent);
                    console.log(`Modified: ${filePath}`);
                } else {
                    console.log(`No modifications: ${filePath}`);
                }
            }

            console.log('Processing complete.');
        }
    } catch (error) {
        console.error('Error occurred:', error);
    }
}

function removeSequences(content) {
    const lines = content.split('\n');
    let modifiedContent = '';
    let isInsideSequence = false;

    for (const line of lines) {
        if (!isInsideSequence && line.trim().startsWith('Notes:')) {
            isInsideSequence = true;
        }

        if (!isInsideSequence) {
            modifiedContent += line + '\n';
        }

        if (isInsideSequence && (line.trim() === '---' || line.trim() === '---v')) {
            isInsideSequence = false;
            modifiedContent += line
        }
    }

    return modifiedContent.trim();
}

// Example usage
const directoryPath = `${process.argv[3]}`; // Replace this with your directory path
removeMultilineSequences(directoryPath);