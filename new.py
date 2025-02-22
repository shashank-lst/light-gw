import csv
from datetime import datetime, timedelta
from collections import defaultdict

# Function to check if two date ranges overlap
def has_overlap(start1, end1, start2, end2):
    return start1 <= end2 and start2 <= end1

# Function to calculate the duration excluding weekends
def calculate_free_time(start, end):
    free_time = timedelta(0)
    current_date = start

    while current_date < end:
        # Skip weekends (Saturday and Sunday)
        if current_date.weekday() < 5:  # Monday to Friday are weekdays (0-4)
            free_time += timedelta(days=1)
        current_date += timedelta(days=1)
    
    return free_time

# Hardcoded start and end dates for free time calculation
free_period_start = datetime(2025, 3, 4)  # March 4, 2025
free_period_end = datetime(2025, 12, 31)  # December 31, 2025

# Function to read activities from CSV file
def read_activities_from_file(file_path):
    activities = []
    with open(file_path, mode='r') as file:
        csv_reader = csv.DictReader(file)
        
        for row in csv_reader:
            if row:  # To avoid processing empty rows
                activity = {
                    'Activity': row['Activity'],
                    'Start Date': datetime.strptime(row['Start Date'], '%d-%b-%Y'),
                    'End Date': datetime.strptime(row['End Date'], '%d-%b-%Y'),
                    'Assignee': row['Assignee']
                }
                activities.append(activity)
    return activities

# Read activities from the given CSV file
file_path = 'activities.csv'  # Provide the path to your CSV file here
activities = read_activities_from_file(file_path)

# Grouping conflicts by assignee
conflicts = defaultdict(list)

# Iterate through the activities and check for conflicts
for i, activity1 in enumerate(activities):
    for j, activity2 in enumerate(activities):
        if i < j and activity1['Assignee'] == activity2['Assignee']:
            if has_overlap(activity1['Start Date'], activity1['End Date'], activity2['Start Date'], activity2['End Date']):
                conflicts[activity1['Assignee']].append((activity1['Activity'], activity2['Activity']))

# Calculate free time and occupied time for each assignee
assignee_time = defaultdict(lambda: {'occupied': timedelta(0), 'free': timedelta(0)})

# Iterate through all activities and calculate occupied time per assignee
for activity in activities:
    assignee = activity['Assignee']
    start = activity['Start Date']
    end = activity['End Date']
    
    # Calculate the occupied time for each assignee
    assignee_time[assignee]['occupied'] += (end - start)

# Calculate the total free time for each assignee within the defined period
total_free_time = calculate_free_time(free_period_start, free_period_end)

# Calculate the free time and percentages
for assignee in assignee_time:
    assignee_time[assignee]['free'] = total_free_time - assignee_time[assignee]['occupied']
    
    # Calculate the percentage of time occupied and free
    occupied_percentage = (assignee_time[assignee]['occupied'] / total_free_time) * 100
    free_percentage = (assignee_time[assignee]['free'] / total_free_time) * 100

    print(f"\nAssignee: {assignee}")
    print(f"Occupied Time: {assignee_time[assignee]['occupied']}")
    print(f"Free Time: {assignee_time[assignee]['free']}")
    print(f"Occupied Percentage: {occupied_percentage:.2f}%")
    print(f"Free Percentage: {free_percentage:.2f}%")
    
    # Print conflicts if any
    if conflicts[assignee]:
        print(f"Conflicts: {conflicts[assignee]}")
    else:
        print("No Conflicts")
