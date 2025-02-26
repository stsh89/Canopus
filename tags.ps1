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
    "get" {
        $id = Read-Host 'Tag ID'
        $command = $base_command + "Get-Tag --ID $id" + " | ConvertFrom-Json"

        Invoke-Expression $command
    }

    "list" {
        $command = $base_command + "List-Tags"
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
}
