param(
    $CommandName
)

$command = "cargo run -q -p canopus-cli -- "

switch ($CommandName) {
    "delete" {
        $id = Read-Host 'Remark ID'
        $command = $command + "Delete-Remark --ID $id"
    }

    "get" {
        $id = Read-Host 'Remark ID'
        $command = $command + "Get-Remark --ID $id" + " | ConvertFrom-Json"
    }

    "list" {
        $command = $command + "List-Remarks" + " | ConvertFrom-Csv"
        Write-Host $command
    }

    "new" {
        $essence = Read-Host 'Remark essence'
        $command = $command + "New-Remark --Essence $essence"

        do {
            $tag = Read-Host 'Remark tag'

            if ($tag -ne "") {
                $command = $command + " --Tag $tag"
            } else {
                break
            }
        } while ($true)
    }
}

Invoke-Expression $command
