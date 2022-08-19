const fs = require('fs');

const baseDir = "./keys/";

const merge_keys = () => {
    try {
        let files = fs.readdirSync(baseDir);
        let result = "";

        const basicKeys = fs.readFileSync("./program_helper/basic_keys.js", 'utf8');
        result += `${ basicKeys }\n\n`;

        let configs = "\n";
        let pdas    = "\n";
        let pool    = "\n";
        let pools   = "";
        let account = "\n";
        let tokens  = "";
        let atas    = "";
        let etcs    = "";

        files.forEach(item => {
            const itemPath = baseDir + item;
            if (item.includes('_config_pub')) {
                const filtered_name = item.replace("_config_pub.js", "");
                const uppercase_name = filtered_name.toUpperCase();
                const key_name = uppercase_name + "_CONFIG";

                const data = fs.readFileSync(itemPath, 'utf8');
                configs += `export const ${key_name} = new PublicKey(${ data }); \n`;
            } else if (item.includes('_pda_pub')) {
                const filtered_name = (item).replace("_pda_pub.js", "");
                const uppercase_name = filtered_name.toUpperCase();
                const key_name = uppercase_name + "_PDA";

                const data = fs.readFileSync(itemPath, 'utf8');
                pdas += `export const ${key_name} = new PublicKey(${ data }); \n`;
            } else if (item.includes('_pool_pub')) { 
                const filtered_name = (item).replace("_pool_pub.js", "");
                const uppercase_name = filtered_name.toUpperCase();
                const key_name = uppercase_name + "_POOL";

                const data = fs.readFileSync(itemPath, 'utf8');
                pool += `export const ${key_name} = new PublicKey(${ data }); \n`;
            } else if (item.includes('_account_pub')) { 
                const filtered_name = (item).replace("_account_pub.js", "");
                const uppercase_name = filtered_name.toUpperCase();
                const key_name = uppercase_name + "_ACCOUNT";

                const data = fs.readFileSync(itemPath, 'utf8');
                account += `export const ${key_name} = new PublicKey(${ data }); \n`;
            } else if (item.includes('_tokens_pub')) { 
                const data = fs.readFileSync(itemPath, 'utf8');
                tokens += `\n${data} \n`;
            } else if (item.includes('_pools_pub')) { 
                const data = fs.readFileSync(itemPath, 'utf8');
                pools += `\n${data} \n`;
            } else if (item.includes('_ata_pub')) { 
                const data = fs.readFileSync(itemPath, 'utf8');
                atas += `\n${data} \n`;
            }   else { 
                const data = fs.readFileSync(itemPath, 'utf8');
                etcs += `\n${data} \n`;
            }
        });

        result += configs + pdas + pool + account + tokens + pools + atas + etcs;

        fs.writeFileSync('./config.js', result);
    } catch (err) {
        console.log(err)
    }
}

merge_keys();