// renameProject.js

'use strict';

const path = require("path");
const fs = require("fs");
const rimraf = require('rimraf');
const binaryExtensions = ['.png', '.jar'];
const { execSync } = require('child_process');

if(!process.argv[2]) {
    throw console.error('Project name must be provided!\nrun \'npm run rename {project_name}\'');
}
else if(process.argv.length > 3){
    throw console.error('Worng usage!\nrun \'npm run rename {project_name}\'');
}
else {
    let projectName = process.argv[2];
    if(!isValidProjectName(projectName)){
        throw console.error('Project name is invalid.\nProject name cannot contain \'-\' or \'_\'');
    }
    renameProject(projectName);
    //console.log(`project ${projectName} renamed!`);
    // console.log(`preparing rn-nodeify...`);
    // execSync("npm run rn-nodeify");
    // console.log(`installing pods...`);
    // execSync("npm run podinstall");
    // console.log(`fixing modules...`);
    // execSync(`${__dirname}/fix-modules`);
}

function isValidProjectName(projectName) {
    return projectName.match(/^(?![-_])[a-zA-Z0-9]*$/g);
}

function renameProject(projectName) {
    let rootPath = path.join(__dirname, "..");

    walk(rootPath, (absoluteSrcFilePath) => {
        const fileName = path.basename(absoluteSrcFilePath);
        let isDirectory = fs.lstatSync(absoluteSrcFilePath).isDirectory();

        //ignore files and directories
        if (fileName === 'index.ios.js'
            || fileName === 'index.android.js'
            || fileName === 'index.js'
            || fileName === 'App.js'
            || fileName === 'README.md'
            || fileName === 'package-lock.json'
            || absoluteSrcFilePath.includes('.git/')
            || absoluteSrcFilePath.includes('node_modules/')
            || absoluteSrcFilePath.includes('scripts/')
            || absoluteSrcFilePath.includes('.gradle/')
            || absoluteSrcFilePath.includes('android/app/build/')
            || absoluteSrcFilePath.includes('android/build/')) 
        {
            return null; 
        }


        //delete {android: [.iml, .idea/]} and {iOS: [Podfile.lock, Pods/]}
        if(path.extname(absoluteSrcFilePath) === '.iml' || fileName === 'Podfile.lock'){
            fs.unlinkSync(absoluteSrcFilePath);
            return null;
        }
        else if(isDirectory 
            && (fileName === 'Pods' || fileName === '.idea'))
        {
            rimraf(absoluteSrcFilePath, (error) => {});
            return null;
        }

        //rename file/directory
        var relativeFilePath = path.relative(rootPath, absoluteSrcFilePath);
        const extension = path.extname(relativeFilePath);
        const relativeRenamedPath = dotFilePath(relativeFilePath)
            .replace(/SyloConnectedAppTemplate/g, projectName)
            .replace(/SyloConnectedAppTemplate/g, projectName.toLowerCase());
       
        if (relativeFilePath !== relativeRenamedPath && (fileName.match(/SyloConnectedAppTemplate/g) || fileName.match(/SyloConnectedAppTemplate/g))){
            //console.log(`rename path:  ${relativeRenamedPath}`);
            if(fs.existsSync(relativeRenamedPath)){
                throw console.error(`Cannot rename project to \'${projectName}\', project already existed`);
            }
            fs.renameSync(absoluteSrcFilePath, relativeRenamedPath, (error) => {
                if(error){
                    throw error;
                }
            });

            if(isDirectory){
                //return new name for walking through its sub files
                return path.resolve(relativeRenamedPath);
            }
        }       

         //replace file content
        let replacements = {
            SyloConnectedAppTemplate: projectName,
            SyloConnectedAppTemplate: projectName.toLowerCase(),
        };

        if (relativeRenamedPath && !isDirectory && binaryExtensions.indexOf(extension) === -1) {
            const srcPermissions = fs.statSync(relativeRenamedPath).mode;
            let content = fs.readFileSync(relativeRenamedPath, 'utf8');
            Object.keys(replacements).forEach(
            regex =>
                (content = content.replace(
                    new RegExp(regex, 'g'),
                    replacements[regex],
                )),
            );

            fs.writeFileSync(relativeRenamedPath, content, {
                encoding: 'utf8',
                mode: srcPermissions,
            });
        }

        return path.resolve(relativeRenamedPath);;
    });
}

function dotFilePath(path) {
    if (!path) {
      return path;
    }
    return path
      .replace('_gitignore', '.gitignore')
      .replace('_gitattributes', '.gitattributes')
      .replace('_babelrc', '.babelrc')
      .replace('_flowconfig', '.flowconfig')
      .replace('_buckconfig', '.buckconfig')
      .replace('_watchmanconfig', '.watchmanconfig');
}


/**
 * walk through all sub files and directories for given file
 * @param {*} current root path to start walking
 * @param {*} callback callback with walked file path,
 *                     callnack should always return:
 *                         - a file path for walking through its sub fils. If file name changed from callback, the new file name should be returnd
 *                         - Or return null to skip this file and all its sub files and directories
 */
function walk(current, callback) {
    current = callback(current);
    if(current && fs.lstatSync(current).isDirectory()){
        fs.readdirSync(current).map(child => {
            child = path.join(current, child);
            walk(child, callback);
        });
    }
  
}
