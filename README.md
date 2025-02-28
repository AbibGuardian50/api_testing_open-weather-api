How to Clone and Run the Project

    Open Katalon Studio
    Click the Git icon
    Select Clone Project
    Fill in authentication details:
        Username: Your GitHub username
        Password: Use a Personal Access Token (PAT) instead of a password
            Generate your PAT here https://github.com/settings/tokens
    Click Next
    Choose the desired branch:
        If the repository does not contain multiple branches, simply click Next
    Select the destination directory for your project
    Click Finish

How to Generate and Export Test Reports

    Run the Test Suites and ensure all tests pass successfully
    Navigate to the Reports section
    Select the desired test report
    Right-click on the report file
    Choose Export As → Select the preferred format (e.g., HTML, PDF, etc.)
    Choose the destination folder for the exported report
    Open the exported file to review the results
    
Folder Structure Overview

    .cache/: This directory typically stores temporary files and cache data to optimize performance during test execution.
    
    Include/: Contains configuration files and custom keywords that extend Katalon's built-in functionalities.
    
    Object Repository/: Houses saved objects from Postman, which can be utilized in Katalon for API requests.
    
    Profiles/: Stores environment-specific variables, allowing for parameterization and management of different testing environments.
    
    Scripts/: Contains the underlying scripts for test cases, typically auto-generated by Katalon.
    
    Test Cases/: Includes individual test cases that define specific testing scenarios.
    
    Test Suites/: Groups multiple test cases to be executed together, facilitating organized and efficient testing.
    
    settings/: Holds project-specific settings and configurations.

Source to find Lat & long 

    https://www.findlatitudeandlongitude.com/l/Jakarta+Selatan%2C+DKI+Jakarta/3759633/#google_vignette
