export interface Experience extends ExperienceDto {
  id: number;
  userId: number;
}

export interface ExperienceDto {
  companyName: string;
  occupation: string;
  locationName?: string;
  locationType: string;
  employmentType: string;
  startDate: Date;
  endDate?: Date;
  isCurrent: boolean;
  description: string;
}
