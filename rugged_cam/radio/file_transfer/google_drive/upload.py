from pydrive.auth import GoogleAuth
from pydrive.drive import GoogleDrive

# TODO: avoid authenticating via oauth if no human in loop
# TODO: handle no internet connection or hanging request
def upload_file(file_path, authorize_if_needed=False):
    gauth = GoogleAuth()
    # Try to load saved client credentials
    gauth.LoadCredentialsFile("client_creds.txt")
    # dont try to authorize with web sign in, if not asked to 
    if gauth.credentials is None and authorize_if_needed == False:
        return
    if gauth.credentials is None and authorize_if_needed:
        # Authenticate if they're not there
        gauth.LocalWebserverAuth()
    elif gauth.access_token_expired:
        # Refresh them if expired
        gauth.Refresh()
    else:
        # Initialize the saved creds
        gauth.Authorize()
    # Save the current credentials to a file
    gauth.SaveCredentialsFile("client_creds.txt")
    drive = GoogleDrive(gauth)
    # now upload file
    folder_id = "1C0-LZJwx-JqJvWVZjaygHyMItg0va-VE"
    file = drive.CreateFile({"parents": [{"kind": "drive#fileLink", "id": folder_id}]})
    file.SetContentFile(file_path)
    file.Upload()





