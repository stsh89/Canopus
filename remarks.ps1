param(
    $CommandName
)

$base_command = "cargo run -q -p canopus-cli -- "

function LoadNextPage {
    Write-Host "Press Enter to load next page or Esc to exit..."

    while ($true) {
        $key = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

        if ($key.VirtualKeyCode -eq 13) {  # Enter key
            return $true
        } elseif ($key.VirtualKeyCode -eq 27) {  # Escape key
            return $false
        }
    }
}

switch ($CommandName) {
    "delete" {
        $id = Read-Host 'Remark ID'
        $command = $base_command + "Delete-Remark --ID $id"

        Invoke-Expression $command
    }

    "get" {
        $id = Read-Host 'Remark ID'
        $command = $base_command + "Get-Remark --ID $id" + " | ConvertFrom-Json"

        Invoke-Expression $command
    }

    "list" {
        $command = $base_command + "List-Remarks"
        $listing_command = $command + " | ConvertFrom-Csv"
        $rows = Invoke-Expression $listing_command

        do {
            if ($rows.Count -eq 0) {
                return
            }

            $rows | Format-Table

            if (-not(LoadNextPage)) {
                return
            }

            $listing_command = $command + " --LoadNextPage | ConvertFrom-Csv"
            $rows = Invoke-Expression $listing_command
        } while($true)
    }

    "new" {
        $essence = Read-Host 'Remark essence'
        $command = $base_command + "New-Remark --Essence $essence"

        do {
            $tag = Read-Host 'Remark tag'

            if ($tag -ne "") {
                $command = $command + " --Tag $tag"
            } else {
                break
            }
        } while ($true)

        $new_command = $command + " | ConvertFrom-Json"

        Invoke-Expression $new_command
    }

    "update" {
        $id = Read-Host 'Remark ID'
        $command = $base_command + "Update-Remark --ID $id"

        $essence = Read-Host 'Remark essence'

        if ($essence -ne "") {
            $command = $command + " --Essence $essence"
        }

        do {
            $tag = Read-Host 'Add remark tag'

            if ($tag -ne "") {
                $command = $command + " --AddTag $tag"
            } else {
                break
            }
        } while ($true)

        do {
            $tag = Read-Host 'Delete remark tag'

            if ($tag -ne "") {
                $command = $command + " --RemoveTag $tag"
            } else {
                break
            }
        } while ($true)

        $new_command = $command + " | ConvertFrom-Json"

        Invoke-Expression $new_command
    }
}
