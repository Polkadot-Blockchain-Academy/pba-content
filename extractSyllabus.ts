const { globSync } = require('glob')
const { writeFileSync, readFileSync } = require('fs')

const extractTitleFromMarkdown = (filePath: string): string => {
  try {
    const content = readFileSync(filePath, 'utf8')
    const frontmatterMatch = content.match(/^---\s*\n([\s\S]*?)\n---\s*\n/)
    
    if (frontmatterMatch) {
      const frontmatter = frontmatterMatch[1]
      const titleMatch = frontmatter.match(/^title:\s*(.+)$/m)
      if (titleMatch) {
        return titleMatch[1].trim()
      }
    }
  } catch (error) {
    console.warn(`Warning: Could not read file ${filePath}:`, (error as Error).message)
  }
  
  // Fallback to filename processing if no title found
  const filename = filePath.split('/').pop() || ''
  return filename
    .replace('_slides.md', '')
    .replace('_Slides.md', '')
    .replace('-slides.md', '')
    .replace('-Slides.md', '')
    .replace(/\d+-(.+)/, '$1') // Remove leading numbers and dash
    .replace(/-/g, ' ')
    .replace(/_/g, ' ')
}

// Sorting based on the initial number (even if it is in a string)
const sortArray = (arr: string[]) => {
  const tempArr: any[] = []
  let n
  for (let i in arr) {
    tempArr[i] = arr[i].match(/([^0-9]+)|([0-9]+)/g)
    for (let j in tempArr[i]) {
      if (!isNaN((n = parseInt(tempArr[i][j])))) {
        tempArr[i][j] = n
      }
    }
  }
  tempArr.sort(function (x, y) {
    for (let i in x) {
      if (y.length < i || x[i] < y[i]) {
        return -1 // x is longer
      }
      if (x[i] > y[i]) {
        return 1
      }
    }
    return 0
  })
  for (let i in tempArr) {
    arr[i] = tempArr[i].join('')
  }
  return arr
}

const files = globSync('./syllabus/**/**/*lides.md', { ignore: ['node_modules/**', '**/README.md'] })
const sortedFiles = sortArray([...files])

const fileData: { [key: string]: string } = {}
sortedFiles.forEach((filePath: string) => {
  const relativePath = filePath.replace('./syllabus/', '')
  const title = extractTitleFromMarkdown(filePath)
  fileData[relativePath] = title
})

const parsed: any = {}
Object.entries(fileData).forEach(([filePath, title]) => {
  const pathWithoutExtension = filePath
    .replace('_slides.md', '')
    .replace('_Slides.md', '')
    .replace('-slides.md', '')
    .replace('-Slides.md', '')
  
  let position = parsed
  const split = pathWithoutExtension.split('/')
  
  for (let j = 0; j < split.length; j++) {
    if (split[j] !== '') {
      if (j === split.length - 1) {
        position[split[j]] = title
      } else {
        if (typeof position[split[j]] === 'undefined') {
          position[split[j]] = {}
        }
        position = position[split[j]]
      }
    }
  }
})

const data = JSON.stringify(parsed, null, 2)
writeFileSync('./src/syllabus.json', data)
